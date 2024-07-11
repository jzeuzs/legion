%% -*- erlang -*-

%% Main function that gets executed when the script runs
main(_Args) ->
    %% Read and print input from stdin
    loop().

%% Loop to continuously read lines from stdin until eof
loop() ->
    case io:get_line('') of
        eof -> ok;  %% End of input
        Line -> 
            io:format("~s", [Line]),  %% Print the line
            loop()  %% Continue the loop
    end.
