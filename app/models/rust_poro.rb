require 'ffi'

class RustPoro
  extend FFI::Library

  if Rails.env == "development"
    ffi_lib_path = Rails.root.join(
      'bin',
      'embedded',
      'rust',
      'target',
      'release',
      'libnth_prime.dylib'
    )
  else
    ffi_lib_path = Rails.root.join(
      '../',
      'bin',
      'embedded',
      'rust',
      'target',
      'release',
      'libnth_prime.dylib'
    )
  end

  ffi_lib ffi_lib_path
  attach_function :nth_prime, [:int], :int
end
