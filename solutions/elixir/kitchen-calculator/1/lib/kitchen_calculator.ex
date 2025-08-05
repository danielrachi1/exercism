defmodule KitchenCalculator do
  def get_volume(volume_pair) do
    elem(volume_pair, 1)
  end

  def to_milliliter({:cup, volume} = _volume_pair) do
    {:milliliter, volume * 240}
  end

  def to_milliliter({:fluid_ounce, volume} = _volume_pair) do
    {:milliliter, volume * 30}
  end

  def to_milliliter({:teaspoon, volume} = _volume_pair) do
    {:milliliter, volume * 5}
  end

  def to_milliliter({:tablespoon, volume} = _volume_pair) do
    {:milliliter, volume * 15}
  end

  def to_milliliter({:milliliter, volume} = _volume_pair) do
    {:milliliter, volume * 1}
  end

  def from_milliliter({:milliliter, volume} = _volume_pair, :cup = unit) do
    {unit, volume/240}
  end

  def from_milliliter({:milliliter, volume} = _volume_pair, :fluid_ounce = unit) do
    {unit, volume/30}
  end

  def from_milliliter({:milliliter, volume} = _volume_pair, :teaspoon = unit) do
    {unit, volume/5}
  end

  def from_milliliter({:milliliter, volume} = _volume_pair, :tablespoon = unit) do
    {unit, volume/15}
  end

  def from_milliliter({:milliliter, volume} = _volume_pair, :milliliter = unit) do
    {unit, volume / 1}
  end

  def convert(volume_pair, unit) do
    from_milliliter(to_milliliter(volume_pair), unit)
  end
end
