# Motley Fool FOLD Dev Interview Challenge <!-- omit in toc -->

## Table of Contents <!-- omit in toc -->

- [How to Install & Run](#how-to-install--run)
- [Evaluation Criteria & Notes](#evaluation-criteria--notes)
  - [Primary Evaluation Criteria](#primary-evaluation-criteria)
  - [Hosted on GitHub](#hosted-on-github)
  - [Retain Original Functionality](#retain-original-functionality)
  - [Link to Project Template](#link-to-project-template)
  - [New Feature & Explanation](#new-feature--explanation)
  - [Highlights that make this project uniquely *mine*](#highlights-that-make-this-project-uniquely-mine)
  - [Areas of Improvement](#areas-of-improvement)
- [Project Prompt](#project-prompt)
  - [General project guidelines](#general-project-guidelines)
  - [Base Project Templates](#base-project-templates)
  - [New Feature Ideas](#new-feature-ideas)
- [Original README](#original-readme)

[Rust]: https://www.rust-lang.org/
[Rocket]: https://rocket.rs/

## How to Install & Run

- clone this repository (or fork and clone, if you'd rather)
- this app is built in [Rust] and [Rocket] and makes use of features in the nightly channel, which means you'll need a recent release of Rust nightly in order to run this Rocket application.
  1. to install the latest version of Rust, I recommend using `rustup`. Install `rustup` by following the instructions on [its website](https://rustup.rs/).
  2. Once `rustup` is installed, ensure the latest toolchain is installed by running the command `rustup default stable`.
  3. Now, since this application uses Nightly, you can either...
     - default to nightly with `rustup default nightly`
     - or simply override within this directory by travelling to the `motley-fold-todo-app` directory in your terminal and running `rustup override set nightly` as described in the [rust-lang docs](https://rust-lang.github.io/rustup/overrides.html#directory-overrides)
- Once all of the above is set up and you've got a nightly installation of Rust ready to roll, `cargo run` to start up the local server!

## Evaluation Criteria & Notes

### Primary Evaluation Criteria

- [x] [Code should be hosted on GitHub]((#hosted-on-github))
- [x] [TODO application retains original functionality]((#retain-original-functionality))
- [x] [TODO application contains 1 new feature](#new-feature--explanation)
- [x] A README with the following elements
  - [x] [Link to project template used](#link-to-project-template)
  - [x] [Explanation of the chosen feature that was implemented](#new-feature--explanation)
  - [x] [Any highlights that make the project uniquely yours](#highlights-that-make-this-project-uniquely-mine)
  - [x] [Possible areas of improvement](#areas-of-improvement)
- [x] Use any publicly available packages that you need
- [x] [Include directions for installing and running your application](#how-to-install--run)
- [x] Have fun! Consider modifications that highlight your development approach

### Hosted on GitHub

[This repository is hosted here](https://github.com/chazkiker2/motley-fold-todo-app)

### Retain Original Functionality

The original functionality is completely maintained, even after updating dependency versions (over the course of three years... more on this below)

### Link to Project Template

- Original project template: <https://gitlab.com/duelinmarkers/todo-backend-rocket-rust/-/tree/master/>
- This template was taken from <http://www.todobackend.com/index.html>. You can find it by clicking on "Rocket" or "Rust" and finding the "Rust / Rocket" entry

### New Feature & Explanation

- The feature that I chose to implement was a search feature which allows users to hit `/search/<search_query>` to search todos by title (case insensitive)
- The endpoint has the route `/search/<search_query>` where `<search_query>>` would be replaced with the term to search.
- The query is a case-insensitive comparison (i.e., a todo item with the title `"TODO"` would be included in the results from a search at the route `/search/todo`)
- The query will match if placed anywhere within the title. (i.e., `/search/od` would match titles `"TODO_001"`, `"ODD_TITLE"`, and `"food"` alike)
- The user will receive a list of all todos with titles that match the given query. If no titles match, then the user would receive an empty list.

### Highlights that make this project uniquely *mine*

- Though I have been using [Rust] the past month or so, I've never used [Rocket] before other than dabbling in a couple of tutorials. (Note: [Rocket] is the framework with which this app is built). Further, I certainly haven't used several of the dependencies used specifically in this application, including: `rocket`, `diesel`, `r2d2`.

  I chose to use Rocket for this challenge b/c I've been wanting to use it and figured it may demonstrate to TMF that ya boy is very comfortable using brand new frameworks :wink:.

- Beyond the fact that Rocket is new to me, I think it's worth noting that the existing codebase used `rocket v0.3.3` which was THREE YEARS old (as was the rust compiler)...

  Now, Rust is a relatively newer language and thus the language itself (and all of its frameworks) change fast and drastically. Three years in Java Spring, for instance, is rather different than three years in Rust Rocket.

  When I initially [cloned this project from the template](https://gitlab.com/duelinmarkers/todo-backend-rocket-rust/-/tree/master/), `cargo build` literally failed to compile the package. What this means is that the codebase from three years ago that once compiled and worked no longer did. `rustc` (the Rust compiler) literally rejected the code in the package. *(Bonus points to Motley Fool if whomever is grading this can clone from [the template](https://gitlab.com/duelinmarkers/todo-backend-rocket-rust/-/tree/master/) and get it working without looking at my changes :wink:)*

  So The first thing I did was update all `rocket` dependencies in the package to the newest stable version. This is not a magic fix, so I also updated several other pieces in the code to get this package in a buildable and runnable state.

  This was definitely new to me, but I learned a thing or two about `rustc`, and I got to see firsthand some of the changes that have been made to `rocket` throughout the years.

  Thankfully, one of Rust's most highly-touted values is its backwards compatibility. This value is upheld throughout the community, from the core of rust-lang throughout all of its favorite crates and frameworks. So this dependency version overhaul, while not as straightforward to perfect as one might think, did allow most of the code to stay the same once a couple things changed!

- This is a rather small and irrelevant detail, but the initial code was in GitLab (rather than GitHub) so I got to learn how to port over a repository from GitLab to GitHub. Super duper simple, but a nice little piece of knowledge to have nonetheless. [Here's how to do it.](https://stackoverflow.com/questions/22265837/transfer-git-repositories-from-gitlab-to-github-can-we-how-to-and-pitfalls-i)

### Areas of Improvement

I see several areas of improvement in this codebase.

1. Though `rustc` REQUIRED an update to the latest rocket dependencies, the `diesel` and `r2d2` dependencies were permitted to stay the same. There are newer versions of each of these, and to keep the code up to best practices I'd personally want to see these packages at the latest version.
2. Rust has improved as a language throughout the past three years with lots of different updates and features. Further, Rust has especially established and homed in on its style and elegance in the past three years. The code found in this codebase is not the idiomatic Rust that most Rustaceans have come to know and love. Lots of repeated code, logic, and unnecessary bulk. I'd be motivated by a hefty refactor that implemented more idiomatic code.
3. There's a LOT of other features that could be added to a Todo App (no wonder TMF picked this as a challenge) and I'd be motivated to add several other features.
4. Documentation is nowhere to be found (except for the functions I added in). I'd most certainly document every endpoint as well as add doc-comments to every function with examples at least in every public function to align with Rust norms.
5. On the `diesel` dependency front: not only are there new versions, but the newest version has changed quite a bit in the past three years. I would want to refactor the use of diesel in this application quite a lot.

## Project Prompt

Hello aspiring Fool!

Below you will find guidelines for a small TODO application. Our hope is that you'll find this project a bit like a blank canvas, you can fill it in however you please to show us something about yourself, thought process, and development style.

### General project guidelines

- 1-2 hours time commitment
- Choose a base project repository from the links below
- Fork/copy the repository to your personal GitHub account
- Use any publicly available packages that you need
- Include directions for installing and running your application
- Have fun! Consider modifications that highlight your development approach

### Base Project Templates

- http://www.todobackend.com/index.html
- https://todomvc.com/

### New Feature Ideas

Here are a few starter ideas for new features. You may also choose to modify or extend these; or choose something entirely different. The world is your oyster!

- [x] Search todos
- [ ] Implement assignee for todo, maybe prioritization too. Listing all todos can sort by assignee/priority
  - [x] prioritization (order)
  - [ ] assignees
- [x] Implement “done” feature, whereby if a todo is done it gets shown in a “done” list
- [ ] Create tags for todo items and ability to view all items by tag name

## Original README

```md
A Todo-Backend using Rocket and Rust
===

- What's this? See http://todobackend.com/.
- [Run the Todo-Backend Specs](https://www.todobackend.com/specs/index.html?https://todo-backend-rocket-rust.herokuapp.com/) against this application
- [How to Deploy a Rocket Application to Heroku](http://www.duelinmarkers.com/2017/10/21/how-to-deploy-a-rocket-application-to-heroku.html)
```
