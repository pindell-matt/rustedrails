class RustController < ApplicationController

  def show
    rust_poro = RustPoro.new
    @nth_prime = rust_poro.nth_prime(5)
  end

end
