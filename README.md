
<h1 align="center">
  <br>
  <a href="https://plotapi.com"><img src="https://plotapi.com/wp-content/uploads/2021/07/plotapi_logo.svg" alt="Plotapi" width="300"></a>
</h1>

<h4 align="center">Engaging visualisations, made easy.</h4>

<p align="center">
<a href="https://crates.io/crates/chord"><img src="https://img.shields.io/badge/rust%20crate-0.1.0-success.svg"></a>
<a href="https://pypi.org/project/plotapi/"><img src="https://img.shields.io/badge/pypi%20package-6.0.1-success.svg"></a>
<a href="https://plotapi.com/"><img src="https://img.shields.io/badge/license-get-green.svg"></a>
<a href="https://discord.polyra.com"><img src="https://img.shields.io/badge/chat-join-7289da.svg"></a>
<a href="https://jupyter.org"><img src="https://img.shields.io/badge/supports-jupyter-orange.svg"></a>
<a href="https://www.linkedin.com/in/shahinrostami/"><img src="https://img.shields.io/badge/linked-in-blue.svg"></a>
</p>

<p align="center">
  <a href="#key-features">Key Features</a> •
  <a href="https://plotapi.com">Get Access</a> •
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#license">License</a>
</p>

![screenshot](https://plotapi.com/wp-content/uploads/2021/08/1500x500.jpeg)

## Key Features

* **Plotapi Chord** - Illustrate inter-relationships between data.
* **Plotapi Sankey** - Illustrate the flow from one set of values to another.
* **Plotapi Terminus** - Illustrate distributing something out amongst recipients.
* **Plotapi Bar Fight** - A beautiful take on the classic Bar Chart Race.
* **Plotapi Pie Fight** - A beautiful take on the classic Pie Chart Race.
* **Plotapi Heat Map** - Beautiful and interactive heat maps.
* **Plotapi Line Fight** - A beautiful take on the classic Line Chart Race.
* **Plotapi Pareto Front** - Illustrate non-dominated (Pareto) fronts over time.
* **Upcoming Visualisations** - Access to new visualisations as they are introduced.
* **Supports Most Programming Languages** - Get started with any language able to make HTTP requests.
* **First-Class Python Support** - Get started with pip install plotapi
* **Jupyter Lab/Notebook + Google Colab Support** - Super-charge your notebooks with inline visualisations.
* **Beautiful Themes & Fonts** - Select from pre-made beautiful themes, or add your own.
* **Share or Embed Interactive HTML** - Download and embed your interactive visualisations.
* **Render to PDF, SVG, or PNG** - Generate high-quality output ready for print.
* **Record Animation to Video** - Create social media-ready animated visualisations.

## Get Access

Visit [the website](https://plotapi.com/#pricing) to get access to Plotapi.

## Installation

Get up and running with Rust with the `plotapi` crate.

## Usage

### Example - Chord Pro

https://user-images.githubusercontent.com/15690380/126084021-b008b256-2a31-4106-84af-42777ea480d9.mp4

#### Created with Plotapi

```rust
use plotapi::params;
use plotapi::Visualisation;

fn main() {
    let names: Vec<String> = vec!["A", "B", "C", "1", "2", "3"]
        .into_iter()
        .map(String::from)
        .collect();

    let matrix: Vec<Vec<f64>> = vec![
        vec![0.0, 0.0, 0.0, 1.0, 4.0, 1.0],
        vec![0.0, 0.0, 0.0, 1.0, 3.0, 2.0],
        vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0],
        vec![1.0, 1.0, 1.0, 0.0, 0.0, 0.0],
        vec![4.0, 3.0, 2.0, 0.0, 0.0, 0.0],
        vec![1.0, 2.0, 2.0, 0.0, 0.0, 0.0],
    ];

    let colors: Vec<String> = vec![
        "#7400B8", "#5E60CE", "#5684D6", "#56CFE1", "#64DFDF", "#80FFDB",
    ]
    .into_iter()
    .map(String::from)
    .collect();

    let param = params!({
        "matrix": matrix,
       "names": names,
       "colors":  colors

    });

    Visualisation {
        api_key: "17ec2f26-076c-4110-a23a-9a02efe2d52a",
        params: param,
        endpoint: "chord",
    }
    .to_html();
}

```

### Example - Sankey Pro

https://user-images.githubusercontent.com/15690380/126084745-712fd744-b626-429d-85f3-30b11979fe30.mp4



## License

MIT

---

## Prefer not to code? Check out the App at [PlotAPI.com](https://plotapi.com). Python version available.

<a href="https://plotapi.com"><img src="https://plotpanel.com/static/marketing/plotpanel_preview_1.jpg" alt="PlotPanel"></a>


> [plotapi.com](https://plotapi.com) &nbsp;&middot;&nbsp;
> GitHub [@shahinrostami](https://github.com/shahinrostami) &nbsp;&middot;&nbsp;
> Twitter [@shahinrostami](https://twitter.com/shahinrostami)  &nbsp;&middot;&nbsp;
> A [Polyra](https://polyra.com) innovation
