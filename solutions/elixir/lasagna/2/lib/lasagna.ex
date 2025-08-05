defmodule Lasagna do
  def expected_minutes_in_oven() do
    40
  end

  def remaining_minutes_in_oven(current_minutes_in_oven) do
    expected_minutes_in_oven() - current_minutes_in_oven
  end

  def preparation_time_in_minutes(added_layers) do
    2 * added_layers
  end

  def total_time_in_minutes(added_layers, current_minutes_in_oven) do
    preparation_time_in_minutes(added_layers) + current_minutes_in_oven
  end

  def alarm() do
    "Ding!"
  end
end
