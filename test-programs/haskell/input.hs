main :: IO ()
main = do
    contents <- getContents
    putStr contents
