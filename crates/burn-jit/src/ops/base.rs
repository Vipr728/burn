use crate::{element::JitElement, kernel, tensor::JitTensor, JitRuntime};
use burn_cube::CubeElement;
use burn_tensor::{Shape, TensorData};
use std::marker::PhantomData;

pub(crate) fn from_data<R: JitRuntime, E: JitElement, const D: usize>(
    data: TensorData,
    device: &R::Device,
) -> JitTensor<R, E, D> {
    let shape: Shape<D> = (&data.shape).into();
    let client = R::client(device);
    let buffer = client.create(data.convert::<E>().as_bytes());

    JitTensor::new_contiguous(client, device.clone(), shape, buffer)
}

pub(crate) async fn into_data<R: JitRuntime, E: JitElement, const D: usize>(
    tensor: JitTensor<R, E, D>,
) -> TensorData {
    let tensor = kernel::into_contiguous(tensor);

    let bytes = tensor.client.read_async(tensor.handle.binding()).await;
    TensorData::new(E::from_bytes(&bytes).to_vec(), tensor.shape)
}

pub(crate) async fn bool_into_data<R: JitRuntime, const D: usize>(
    tensor: JitTensor<R, u32, D>,
) -> TensorData {
    let tensor = kernel::into_contiguous(tensor);
    let bytes = tensor.client.read_async(tensor.handle.binding()).await;
    TensorData::new(
        u32::from_bytes(&bytes).iter().map(|i| *i != 0).collect(),
        tensor.shape,
    )
}

pub(crate) fn to_device<R: JitRuntime, E: JitElement, const D: usize>(
    tensor: JitTensor<R, E, D>,
    device: &R::Device,
) -> JitTensor<R, E, D> {
    if &tensor.device == device {
        return tensor;
    }

    let client = R::client(device);
    tensor.to_client(client, device.clone())
}

pub(crate) fn empty<R: JitRuntime, E: JitElement, const D: usize>(
    shape: Shape<D>,
    device: &R::Device,
) -> JitTensor<R, E, D> {
    let client = R::client(device);
    let buffer = client.empty(shape.num_elements() * core::mem::size_of::<E>());

    JitTensor::new_contiguous(client, device.clone(), shape, buffer)
}

pub(crate) fn swap_dims<R: JitRuntime, E: JitElement, const D: usize>(
    mut tensor: JitTensor<R, E, D>,
    dim1: usize,
    dim2: usize,
) -> JitTensor<R, E, D> {
    tensor.strides.swap(dim1, dim2);
    tensor.shape.dims.swap(dim1, dim2);

    tensor
}

pub(crate) fn permute<R: JitRuntime, E: JitElement, const D: usize>(
    mut tensor: JitTensor<R, E, D>,
    axes: [usize; D],
) -> JitTensor<R, E, D> {
    // remap strides
    tensor.strides = axes.map(|i| tensor.strides[i]);

    // remap shape
    tensor.shape.dims = axes.map(|i| tensor.shape.dims[i]);

    tensor
}
pub(crate) fn expand<R: JitRuntime, E: JitElement, const D: usize, const D_OUT: usize>(
    tensor: JitTensor<R, E, D>,
    target_shape: Shape<D_OUT>,
) -> JitTensor<R, E, D_OUT> {
    // Initialize new strides with zeros
    let mut new_strides = [0usize; D_OUT];

    // Calculate the difference in dimensions
    let dim_diff = D_OUT.saturating_sub(D);

    // Compare dimensions from the end, setting strides for matching dimensions or broadcasted ones
    let mut tensor_dim_iter = tensor.shape.dims.iter().rev();
    for i in (0..D_OUT).rev() {
        if i >= dim_diff {
            if let Some(&tensor_dim) = tensor_dim_iter.next() {
                if tensor_dim == target_shape.dims[i] || tensor_dim == 1 {
                    // Copy stride for non-broadcast dimensions or set to 0 for broadcast ones
                    new_strides[i] = if tensor_dim == target_shape.dims[i] {
                        tensor.strides[i - dim_diff]
                    } else {
                        0
                    };
                } else {
                    // Error handling: Dimension mismatch for broadcasting
                    panic!(
                        "Dimension mismatch: cannot broadcast dimension {} of tensor to target shape",
                        tensor_dim
                    );
                }
            } else {
                // If the input tensor has fewer dimensions, treat missing dimensions as 1
                // and set stride to 0 (broadcasting)
                new_strides[i] = 0;
            }
        } else {
            // For extra dimensions in the target shape, set stride to 0 (broadcasting)
            new_strides[i] = 0;
        }
    }

    JitTensor {
        client: tensor.client,
        device: tensor.device,
        shape: target_shape,
        strides: new_strides,
        handle: tensor.handle,
        elem: PhantomData,
    }
}

pub(crate) fn reshape<R: JitRuntime, E: JitElement, const D1: usize, const D2: usize>(
    tensor: JitTensor<R, E, D1>,
    shape: Shape<D2>,
) -> JitTensor<R, E, D2> {
    // TODO: Not force standard layout all the time (improve performance).
    let tensor = kernel::into_contiguous(tensor);

    JitTensor::new_contiguous(tensor.client, tensor.device, shape, tensor.handle)
}
