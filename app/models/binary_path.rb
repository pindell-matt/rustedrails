class BinaryPath
  def rust_binary
    if Rails.env == "development" || Rails.env == "test"
      Rails.root.join(
        'bin',
        'embedded',
        'rust',
        'target',
        'release',
        'libnth_prime.dylib'
      )
    else
      'bin/embedded/rust/target/release/libnth_prime.so'
    end
  end
end
