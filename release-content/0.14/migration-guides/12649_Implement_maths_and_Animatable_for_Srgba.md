The previously existing implementation of mul/div for `Srgba` did not modify `alpha` but these operations do modify `alpha` now. Users need to be aware of this change.
