Rails.application.routes.draw do

  get '/nth_prime', to: 'rust#show'

end
