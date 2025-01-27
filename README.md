# rust-mlops
Rust for MLOPs, examples and learnings from my studies and experience with ML tools.
- Workflows that go beyond Jupyter, Conda, Pandas, Numpy, Sklearn stack for mlops.
-  Docker + pip + virtualenv
-  Microservices

```
rustc --version
```

![image](https://github.com/user-attachments/assets/da663ab9-907d-4141-93d9-8f6dce1984af)
Image sourced from: https://github.com/noahgift/rust-mlops-template
# Run with GitHub Actions
GitHub Actions uses a Makefile to simplify automation
To run everything locally do: make all.
```
name: Rust CI/CD Pipeline
on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]
env:
  CARGO_TERM_COLOR: always
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v1
    - uses: actions-rs/toolchain@v1
      with:
          toolchain: stable
          profile: minimal
          components: clippy, rustfmt
          override: true
    - name: update linux
      run: sudo apt update 
    - name: update Rust
      run: make install
    - name: Check Rust versions
      run: make rust-version
    - name: Format
      run: make format
    - name: Lint
      run: make lint
    - name: Test
      run: make test
```

# MLOps project ideas
- CLI query Hugging Face dataset
- CLI summarise News
- Microservice Web Framework - actix
- Microservice Web Framework deploys pre-trained model
-CLI with descriptive statistics on a well known dataset using https://www.pola.rs/[Polars]
- Train a model with PyTorch (via Rust bindings)
- Explore use-cases in Financial Analysis, trading and DeFi
