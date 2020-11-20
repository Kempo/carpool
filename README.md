# Rust Carpool Implementation

A Rust implementation of a modified [Gale-Shapley]("https://en.wikipedia.org/wiki/Gale%E2%80%93Shapley_algorithm") algorithm that finds stable matches between joinable groups.

In this case, our program finds stable matches between a group of drivers, with their preset car spaces, and a group of riders. Each group has their preferences and so we sort who rides with who based on a **stable match**.
