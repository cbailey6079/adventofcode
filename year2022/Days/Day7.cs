namespace year_2022.Days;


public class Day7
{
    private Directory _input = new ("/", null);
    private List<int> _output = new();
    
    public string Part1()
    {
        ParseInput("day7-1");

        DoStuff(_input);
        
        return _output.Sum().ToString();
    }

    public string Part2()
    {
        ParseInput("day7-1");
        
        var spaceNeeded = 30000000 - (70000000 - DoStuff2(_input));

        _output.Sort();
        
        return _output.First(x => x >= spaceNeeded).ToString();
    }

    public string Parttest()
    {
        ParseInput("day7-0");
        
        var spaceNeeded = 30000000 - (70000000 - DoStuff2(_input));

        _output.Sort();
        
        return _output.First(x => x >= spaceNeeded).ToString();
    }

    int DoStuff(Directory dir)
    {
        var subDir = 0;
        foreach (var childDir in dir.Dirs)
        {
           subDir += DoStuff(childDir);
        }

        var dirSize = dir.Files.Sum() + subDir;
        if (dirSize < 100000)
        {
            _output.Add(dirSize);
        }

        return dirSize;
    }
    
    int DoStuff2(Directory dir)
    {
        var subDir = 0;
        foreach (var childDir in dir.Dirs)
        {
            subDir += DoStuff2(childDir);
        }

        var dirSize = dir.Files.Sum() + subDir;
        _output.Add(dirSize);
        
        return dirSize;
    }

    void ParseInput(string name)
    {
        var currentDir = _input;
        foreach (string line in File.ReadLines(@"/home/clayton/Code/adventofcode/year2022/Inputs/" + name + ".txt"))
        {
            int isInt;
            var firstChar = line.Substring(0, 1);
            
            // check for commands
            if (firstChar == "$")
            {
                var command = line.Split(" ");

                if (command[1] == "cd")
                {
                    if (command[2] == "..")
                    {
                        currentDir = currentDir.Parent!;
                        
                    } else if (command[2] != "/")
                    {
                        currentDir = currentDir.Dirs.First(dir => dir.Name == command[2]);
                    }
                }
            }
            
            Int32.TryParse(firstChar, out isInt);
            // check for files
            if (isInt > 0)
            {
                var fileSize = line.Split(" ");
                
                currentDir.Files.Add(Int32.Parse(fileSize[0]));
            } 
            
            // check for DIRSSSSS
            if (firstChar == "d") {
                var dirName = line.Split(" ");
                
                currentDir.Dirs.Add(new(dirName[1], currentDir));
            }
        }  
    }
}

class Directory
{
    public readonly string Name;
    public readonly Directory? Parent;
    public List<int> Files;
    public List<Directory> Dirs;

    public Directory(string name, Directory? parent)
    {
        Name = name;
        Parent = parent;
        Files = new();
        Dirs = new();
    }
}