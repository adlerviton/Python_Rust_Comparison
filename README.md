# Python vs. Rust Comparison
[![Python CI/CD](https://github.com/adlerviton/Python_Rust_Comparison/actions/workflows/PythonActions.yml/badge.svg)](https://github.com/adlerviton/Python_Rust_Comparison/actions/workflows/PythonActions.yml)
[![Rust CI/CD](https://github.com/adlerviton/Python_Rust_Comparison/actions/workflows/RustActions.yml/badge.svg)](https://github.com/adlerviton/Python_Rust_Comparison/actions/workflows/RustActions.yml)


Efficiency Comparison between python and rust for Data Science Applications

## Components:
- S&P 500 historical dataset
- A Makefile
- A Dockerfile
- GitHub Actions for Rust and Python
- Requirements.txt to handle Python dependencies
- Cargo.toml to handle Rust dependencies
- Rust and Python main scripts
- Python load function that loads the dataset to a SQLite DB
- Tests for both Rust and Python

## Objective
This project aims to showcase a simple example of the efficiency of Rust compared to Python for data science applications.

## Rust Results
To Run: make all

Workflow:
- Format code with Cargo Fmt
- Lint code with Clippy
- Test
- Run Main.rs

Output:

<img width="940" alt="Rust" src="https://github.com/adlerviton/Python_Rust_Comparison/blob/main/Screenshot%202023-10-22%20at%209.52.18%20PM.png">

## Python Results
To Run: make python_all

Workflow:
- Install dependencies
- Lint code with pylint
- Format code with black
- Test
- Run Main.py

Output:

<img width="940" alt="Python" src="https://github.com/adlerviton/Python_Rust_Comparison/blob/main/Screenshot%202023-10-22%20at%209.51.19%20PM.png">

## Comparison
Steps Completed:
1. Read SPX.csv file
2. Produce descriptive statistics
3. Count number of lines in the dataset
4. Sum total volume traded over the datasets observations

Speed:
- Python took 0.0054073333740234375 seconds to complete
- Rust took 0.39047 seconds to complete

Memory:
- Python used 7.984375 MB of memory to complete
- Rust used 0 MB of memory to complete

## Summary
In this example, Rust actually took longer to complete but used significantly less memory than Python.

