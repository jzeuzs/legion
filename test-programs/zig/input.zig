const std = @import("std");

pub fn main() !void {
    const stdin = std.io.getStdIn().reader();
    const stdout = std.io.getStdOut().writer();

    var line: [1024]u8 = undefined;
    while (true) {
        const len = try stdin.readUntilDelimiterOrEof(&line, '\n');
        if (len == 0) break;
        try stdout.writeAll(&line[0..len]);
    }
}
