defmodule PaintByNumber do
  def palette_bit_size(color_count) do
    get_bit_size(color_count, 1)
  end

  defp get_bit_size(color_count, count) do
    if Integer.pow(2, count) >= color_count do
      count
    else
      get_bit_size(color_count, count + 1)
    end
  end

  def empty_picture() do
    <<>>
  end

  def test_picture() do
    <<0::2, 1::2, 2::2, 3::2>>
  end

  def prepend_pixel(picture, color_count, pixel_color_index) do
    <<pixel_color_index::size(palette_bit_size(color_count)), picture::bitstring>>
  end

  def get_first_pixel(picture, color_count) do
    if  picture == <<>> do
      nil
    else 
      bit_count = palette_bit_size(color_count)
      <<value::size(bit_count), rest::bitstring>> = <<picture::bitstring>>
      value
    end
  end

  def drop_first_pixel(picture, color_count) do
    if  picture == <<>> do
      ""
    else 
      bit_count = palette_bit_size(color_count)
      <<value::size(bit_count), rest::bitstring>> = <<picture::bitstring>>
      rest
    end
  end

  def concat_pictures(picture1, picture2) do
    <<picture1::bitstring, picture2::bitstring>>
  end
end
