defmodule RPG.CharacterSheet do
  @moduledoc """
  Helps a player fill their character sheet
  """

  @doc "Welcomes the player"
  @spec welcome() :: :ok
  def welcome() do
    IO.puts("Welcome! Let's fill out your character sheet together.")
  end

  @doc "Ask for the caracter name and process the incoming string."
  @spec ask_name() :: String.t()
  def ask_name() do
    IO.gets("What is your character's name?\n")
    |> String.trim()
  end

  @doc "Ask for the character class and process the incoming string."
  @spec ask_class() :: String.t()
  def ask_class() do
    IO.gets("What is your character's class?\n")
    |> String.trim()
  end

  @doc "Ask for the character level, process the string and turn into an integer."
  @spec ask_level() :: non_neg_integer()
  def ask_level() do
    IO.gets("What is your character's level?\n")
    |> String.trim()
    |> String.to_integer()
  end

  @doc "Ask for everything, print it and return it."
  @spec run() :: map()
  def run() do
    welcome()
    %{name: ask_name(), class: ask_class(), level: ask_level()}
    |> IO.inspect(label: "Your character")
  end
end
