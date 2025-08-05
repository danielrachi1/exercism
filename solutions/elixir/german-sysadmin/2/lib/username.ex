defmodule Username do
  @moduledoc """
  Functions to sanitize the usernames for compatibility with the
  old e-mail client
  """

  @doc """
  Process a username so it only returns lowercase latin letter and underscores
  """
  @spec sanitize(char_list()) :: char_list()
  def sanitize([]), do: []

  def sanitize([char | rest]) when char >= ?a and char <= ?z do
    [char | sanitize(rest)]
  end

  def sanitize([char | rest]) when char == ?_ do
    [char | sanitize(rest)]
  end

  def sanitize([char | rest]) when char < ?a or char > ?z do
    case char do
      ?ä -> [?a, ?e | sanitize(rest)]
      ?ö -> [?o, ?e | sanitize(rest)]
      ?ü -> [?u, ?e | sanitize(rest)]
      ?ß -> [?s, ?s | sanitize(rest)]
      _ -> sanitize(rest)
    end
  end
end
