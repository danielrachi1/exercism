defmodule DNA do
  def encode_nucleotide(code_point) do
    case code_point do
      ?\s -> 0b0000
      ?A -> 0b0001
      ?C -> 0b0010
      ?G -> 0b0100
      ?T -> 0b1000
    end
  end

  def decode_nucleotide(encoded_code) do
    case encoded_code do
      0b0000 -> ?\s
      0b0001 -> ?A
      0b0010 -> ?C
      0b0100 -> ?G
      0b1000 -> ?T
    end
  end

  def encode(dna) do
    do_encode(dna, <<>>)
  end

  defp do_encode([], encoded_bitstring), do: encoded_bitstring
  defp do_encode([head | tail], encoded_bitstring) do
    do_encode(tail, <<encoded_bitstring::bitstring, encode_nucleotide(head)::4>>)
  end

  def decode(dna) do
    do_decode(dna, [])
  end

  defp do_decode(<<>>, decoded), do: decoded
  defp do_decode(dna, decoded) do
    <<value::4, rest::bitstring>> = dna
    do_decode(rest, decoded ++ [decode_nucleotide(value)])
  end
end
