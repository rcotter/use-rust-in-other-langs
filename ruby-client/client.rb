require 'ffi'

module Calculations
  extend FFI::Library
  ffi_lib '../calculations/target/release/libcalculations.dylib'
  attach_function :distance_between_two_points_on_the_earth, [], :void
  attach_function :add, [:int, :int], :int
end

Calculations.distance_between_two_points_on_the_earth
puts "2 + 3 = #{Calculations.add(2, 3)}"
puts 'done!'