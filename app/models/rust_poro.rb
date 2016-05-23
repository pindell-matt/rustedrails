class RustPoro
  extend FFI::Library

  @ffi_lib_path = BinaryPath.new.rust_binary

  ffi_lib @ffi_lib_path

  attach_function :nth_prime, [:int], :int
  attach_function :read_json, [:string], :string
end
