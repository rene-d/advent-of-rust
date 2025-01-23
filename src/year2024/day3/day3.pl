#!/usr/bin/env perl

use strict;
use warnings;


# Part 1
sub part1 {
    my ($data) = @_;
    my $sum = 0;
    while ($data =~ /mul\((\d+),(\d+)\)/g) {
        $sum += $1 * $2;
    }
    return $sum;
}

# Part 2
sub part2 {
    my ($data) = @_;
    my $enabled = 1;
    my $total_sum = 0;

    while ($data =~ /(mul\((\d+),(\d+)\)|don't\(\)|do\(\))/g) {
        my $match = $1;
        if ($match eq 'do()') {
            $enabled = 1;
        } elsif ($match eq "don't()") {
            $enabled = 0;
        } elsif ($enabled && $match =~ /mul\((\d+),(\d+)\)/) {
            $total_sum += $1 * $2;
        }
    }

    return $total_sum;
}


my $filename = "input.txt";
if (@ARGV) {
    $filename = $ARGV[0];
}

my $data = '';
{
    local $/ = undef;
    open my $fh, '<', $filename or die "Could not open file '$filename': $!\n";
    $data = <$fh>;
    close $fh;
}
chomp($data);

print part1($data) . "\n";
print part2($data) . "\n";
