! Test program for comment_units
program test_comment_units1
  use m_util

  ! Comments with and without space after '!'
  !No implicit variables
  implicit none

  ! For now just real (double) variables:
  real(8) :: velocity
  real(8) :: length1, length2, duration

  length1 = 10.0 ! [m]
  length1=16.7![m]
  time = 5.0 ! [s]

  velocity = length1 / time ! [m/s]

  length2 = velocity + length1 !should give an error

  print("length: ", length)
  print("time: ", time)
  print("velocity: ", velocity)
end program test_comment_unit
