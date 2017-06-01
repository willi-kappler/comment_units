program test_comment_unit
  use m_util

  implicit none

  real(8) :: velocity
  real(8) :: length, duration

  length = 10.0
  time = 5.0

  velocity = length / time ! [m/s]

  print("length: ", length)
  print("time: ", time)
  print("velocity: ", velocity)
end program test_comment_unit
