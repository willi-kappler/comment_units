module m_util
  contains

  function circle_area(radius)
    implicit none

    real(8) :: radius
    real(8) :: circle_area

    circle_area = PI * radius * radius

  end function circle_area
end module m_util
