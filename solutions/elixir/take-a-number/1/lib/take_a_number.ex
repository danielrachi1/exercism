defmodule TakeANumber do
  def start() do
    spawn(fn -> loop(0) end)
  end

  def loop(current_state) do
    receive do
      {:report_state, sender_pid} -> 
        send(sender_pid, current_state)
        |> loop()
      {:take_a_number, sender_pid} -> 
        send(sender_pid, current_state + 1)
        |> loop()
      :stop -> 
        nil
      _ -> 
        loop(current_state)
    end
  end
end
