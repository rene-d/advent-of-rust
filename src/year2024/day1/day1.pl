#!/usr/bin/env perl

use strict;
use warnings;

# read input
my $filename = $ARGV[0] // 'input.txt';
open my $fh, '<', $filename or die "Cannot open file $filename: $!\n";
my @lines = <$fh>;
close $fh;
chomp(@lines);

my @left;
my @right;
foreach my $line (@lines) {
    my ($a, $b) = split(/\s+/, $line);
    push @left, $a;
    push @right, $b;
}

# sort arrays
@left = sort { $a <=> $b } @left;
@right = sort { $a <=> $b } @right;

# part 1
my $sum = 0;
for my $i (0..$#left) { $sum += abs($left[$i] - $right[$i]); }
print "$sum\n";

# part 2
my %count;
$count{$_}++ for @right;

$sum = 0;
for my $a (@left) { $sum += $a * ($count{$a} // 0); }
print "$sum\n";
