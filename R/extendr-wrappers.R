# nolint start

# This file was created with the following call:
#   .Call("wrap__make_helloextendr_wrappers", use_symbols = TRUE, package_name = "helloextendr")

#' @docType package
#' @usage NULL
#' @useDynLib helloextendr, .registration = TRUE
NULL

#' Return df to R.
#' @export
polars_to_robj <- function() .Call(wrap__polars_to_robj)

