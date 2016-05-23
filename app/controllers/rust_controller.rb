class RustController < ApplicationController

  def show
    rust_poro     ||= RustPoro.new
    @nth_prime    = rust_poro.nth_prime(10)

    json_test     = ObjectForRust.new.to_json
    @rust_to_ruby = rust_poro.read_json(json_test)
  end

end
