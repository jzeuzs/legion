stream = IO.stream(:stdio, :line)
for line <- stream do
  IO.puts(line)
end
