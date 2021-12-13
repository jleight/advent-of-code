using CommandLine;

namespace AoC;

[UsedImplicitly]
internal class SolutionRunnerArguments
{
    [Option('p', "problem", Required = true, HelpText = "The problem to run.")]
    public string? Problem { get; set; }

    [Option('t', "test", Required = false, HelpText = "Uses the _Test.txt data instead of the full dataset.")]
    public bool UseTestData { get; set; }
}
