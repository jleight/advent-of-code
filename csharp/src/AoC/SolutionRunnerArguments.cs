using CommandLine;

namespace AoC;

[UsedImplicitly]
internal class SolutionRunnerArguments
{
    [Option('p', "problem", HelpText = "The problem to run.")]
    public string? Problem { get; set; }

    [Option('t', "test", HelpText = "Uses the _Test.txt data instead of the full dataset.")]
    public bool UseTestData { get; set; }
}
