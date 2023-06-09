# nolint start

# This file was created with the following call:
#   .Call("wrap__make_helloextendr_wrappers", use_symbols = TRUE, package_name = "helloextendr")

#' @docType package
#' @usage NULL
#' @useDynLib helloextendr, .registration = TRUE
NULL

rpxlog_stdf_to_txt <- function(in_path, out_path) .Call(wrap__rpxlog_stdf_to_txt, in_path, out_path)
rpxlog_sumally <- function(path, ...) .Call(wrap__rpxlog_sumally, path, eval(substitute(alist(...))))
rpxlog_header <- function(path) .Call(wrap__rpxlog_header, path)
rpxlog_dtr <- base::Vectorize(function(path, key) .Call(wrap__rpxlog_dtr, path, key), USE.NAMES = F)
rpxlog_ptr <- base::Vectorize(function(path, key) .Call(wrap__rpxlog_ptr, path, key), USE.NAMES = F)