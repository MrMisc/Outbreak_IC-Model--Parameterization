# Outbreak_Parameterization

 A version of the outbreak model that has been repurposed as a parameterizer in order to fill in the inevitable gaps to estimate parameter values from various scenarios. This model is a modified version of the **Outbreak_IC** model at https://github.com/MrMisc/Outbreak_IC-Model. It uses MSE as an objective function to reduce using an **acceleration** and **decceleration** mechanism to try to obtain the ideal parameter estimate(s) that minimises the discrepancy between the actual values that the user has identified versus the simulation has.

## Use considerations

Basic principles apply here. If you have more unknowns, than you have a equations, you have an [underdetermined problem](https://math.libretexts.org/Bookshelves/Linear_Algebra/Matrix_Algebra_with_Computational_Applications_%28Colbry%29/08%3A_04_In-Class_Assignment_-_Linear_Algebra_and_Python/8.3%3A_Underdetermined_Systems). In contrast, if you have more equations than unknwons, then the [system that you have developed almost always has inconsistent results when constructed with random coefficients](https://en.wikipedia.org/wiki/Overdetermined_system).

Ideally, you would have a good amount of data points to fit to, when hunting for a parameter estimate. Other concerns can arise too. 

Another big issue that a user can come across when trying to estimate a parameter is that the data points that they provide are of a non-ideal temporal scale compared to that of the parameter they are trying to estimate. For example, trying to estimate the **probability of disease transmission via contact for shuffles that occur every 15 minutes** from a set of data points that are weekly records, is not going to be able to produce a precise estimate of this parameter. Ideally, for shorter time resolution parameters, you would use data that is more frequent. In the case of the weekly records, for example, it is much easier to estimate a value for the **recovery rate from the disease**.



Finally, it is important that the steps that the user sets for the algorithm to take, at least be greater than or equal to 1/10th of the range of the parameter values that the user has set the model to allow to explore. Any lower and they risk omitting entire ranges of possible solutions within the provided range.



