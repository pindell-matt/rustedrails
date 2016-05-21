require 'ffi'

class RustPoro
  extend FFI::Library
  ffi_lib_path = Rails.root.join(
    'app',
    'models',
    'embedded',
    'rust',
    'target',
    'release',
    'libnth_prime.dylib'
  )
  ffi_lib ffi_lib_path #'embedded/rust/target/release/libnth_prime.dylib'
  attach_function :nth_prime, [:int], :int
end
