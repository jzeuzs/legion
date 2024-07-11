open System

[<EntryPoint>]
let main argv =
    let rec readAndPrint() =
        match Console.ReadLine() with
        | null -> ()
        | line -> 
            printfn "%s" line
            readAndPrint()
    
    readAndPrint()
    0 // return an integer exit code
