! Test program for comment_units
program test_comment_units1
  use m_util

  ! No implicit variables
  implicit none

  ! For now just real (double) variables:
  real(8) :: velocity
  real(8) :: length, duration

  length = 10.0
  time = 5.0

  velocity = length / time ! [m/s]

  print("length: ", length)
  print("time: ", time)
  print("velocity: ", velocity)
end program test_comment_unit
