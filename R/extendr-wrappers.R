# nolint start

# This file was created with the following call:
#   .Call("wrap__make_helloextendr_wrappers", use_symbols = TRUE, package_name = "helloextendr")

#' @docType package
#' @usage NULL
#' @useDynLib helloextendr, .registration = TRUE
NULL

rpxlog_sumally <- function(path) .Call(wrap__rpxlog_sumally, path)
rpxlog_header <- function(path) .Call(wrap__rpxlog_header, path)
rpxlog_ptr <- base::Vectorize(function(path, key) .Call(wrap__rpxlog_ptr, path, key), USE.NAMES = F)