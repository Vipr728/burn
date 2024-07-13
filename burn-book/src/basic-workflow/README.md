# Guide

This guide will walk you through the process of creating a custom model built with Burn. We will
train a simple convolutional neural network model on the MNIST dataset and prepare it for inference.

<div class="warning">
Make sure you're following along with the correct branch of the git repository. 
The main branch's code may contain breaking changes and may not work with the current release version of burn.
If you are using the main branch's example code to follow along, be sure to modify your cargo.toml file
to: 

 ```toml
[dependencies]
burn = { git = "https://github.com/tracel-ai/burn", features = ["train", "wgpu", "vision"], branch = "main" }
```

</div>
For clarity, we sometimes omit imports in our code snippets. For more details, please refer to the
corresponding code in the `examples/guide` [directory](https://github.com/tracel-ai/burn/tree/main/examples/guide).
We reproduce this example in a step-by-step fashion, from dataset creation to modeling and training
in the following sections. It is recommended to use the capabilities of your IDE or text editor to
automatically add the missing imports as you add the code snippets to your code.

<div class="warning">

Be sure to checkout the git branch corresponding to the version of Burn you are using to follow
this guide.

The current version of Burn is `0.14` and the corresponding branch to checkout is `main`.
</div>

The code for this demo can be executed from Burn's base directory using the command:

```bash
cargo run --example guide
```

## Key Learnings

- Creating a project
- Creating neural network models
- Importing and preparing datasets
- Training models on data
- Choosing a backend
- Using a model for inference
