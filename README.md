# Statistic with Rust

Essential Rust Libraries for Statistics:

* **ndarray**, it's a versatile and powerful crate for handling n-dimensional arrays in Rust, it is inspired by *NumPy* and provides similar functionality.
  * support multi-dimensional arrays with shape and strides.
  * efficient operations: element-wise, broadcasting, and linear algebra operations.
  * various array manipulation methods: slicing, stacking, reshaping, and concatenation.
  * interoperability with BLAS and LAPACK libraries for high-performance linear algegra computations.
* **statrs**, a comprehensive statistics library for Rust that provides a wide range of statistical functions, probability distributions, and other mathematical tools. Features:
  * support for common probability distributions, such as: Normal, Poisson, Bernoulli, and more.
  * various descriptive statistics functions like: mean, variance, standard deviation, skewness, and kurtosis.
  * Hypothesis testing functions, such as *t-test, chi-square, and F-test*.
  * Correlation and regression analysis functions.
* **statis**, library focused on providing efficient implementations of statistical algorithms and data structures in Rust. While not as acomprehensive as *statrs*, *statis* offers a more focused set of tools for specific statistical tasks. Features:
  * Data structures for handling histograms, frecuency tables, and probability mass functions.
  * Support for basic descriptive statistics, such as: mean, median, mode, and standard variation.
  * Efficient implementation of various statistical algorithms: order statistics, kernel density estimation, and k-means clustering.
* **plotly** is a popular library for creating interactive and visually plots in various programming languages. The *plotly* can be rendered in a web browser or saved to a file. Features:
  * Support for various plot types: scatter, line, bar, pie, and more.
  * Customizalbe plot aesthetics, such as colors, markers, and axis labels.
  * Interactive plot features, including zooming, panning, and hover tooltips.
  * Ability to save plots as: HTML, SVG, or PNG files.


To use these libraries in your Rust project, we'll need to add them as dependencies in our project's *Cargo.toml* file. Following is a step-by-step walkthrough on how to set up a new Rust project and install these libraries.

## Create a New Rust Project

Open a terminal or command prompt, create our project on the directory where we want to create, and run the following commands:

* *cargo new <my_statistic_project_name>*, this will create a new Rust project with followgin structure:
        my_statistic_project_name /
              |-> Cargo.toml
              |-> src / 
                  |--> main.rs
* In *Cargo.toml* file add library dependencies. We will see a section called *[dependencies]*. Under this section, add the following lines to include library dependencies:
        [dependencies]
        ndarray   = "<id_version>"
        statrs    = "<id_version>"
        statis    = "<id_version>"
        plotly    = "<id_version>"
  *<id_version>* represent the version of each library. We can check for the latest versions of these libraries on their respective GitHub repositories or by searching for them on *crates.io*.
* *Build and run the project*. Under root directory of the project (*<my_statistic_project>*) run the following command: **cargo build**. This command will download and compile the specific dependencies for our project.
* *Import libraries in Rust code*. Open the *src/main.rs* file in our project directory and **import** the libraries by adding the following lines at the beginning of the file:
  
      extern crate ndarray;
      extern crate statrs;
      extern crate statis;
      extern crate plotly;
