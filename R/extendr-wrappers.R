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
rpxlog_ptr_to_df <- function(path, key) .Call(wrap__rpxlog_ptr_to_df, path, key)
rpxlog_ptr_to_dfs <- base::Vectorize(function(path, key) .Call(wrap__rpxlog_ptr_to_df, path, key), USE.NAMES = F)