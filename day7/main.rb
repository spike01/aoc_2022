class State
  def initialize(root)
    @pwd = root.name
    @dir_stack = []
    @filetree = {root.name => root}
    @reading_output = false
    @output = []
    @all_files = []
  end

  attr_reader :all_files

  def reading_output?
    @reading_output
  end

  def cd(dest)
    if dest == ".."
      @dir_stack.pop
    else
      @dir_stack.push(dest)
    end
    @pwd = dest
  end

  def process_line(line)
    case true # match each case with true === <boolean expression>
    when line.start_with?("$ cd")
      process_output if reading_output?
      dest = line[5..].strip
      cd(dest)
    when line.start_with?("$ ls")
      @reading_output = true
    when reading_output? && !line.start_with?("$")
      @output.push(line.strip)
    end
  end

  def process_output
    @output.each do |o|
      info, name = o.split(" ").map(&:strip)
      file = AocFile.new(name, info)

      cwd = @dir_stack.reduce(@filetree) do |cwd, popd|
        cwd[popd].size += file.size
        cwd[popd].tree
      end

      all_files.push(file)
      cwd[name] = file
    end

    @output = []
    @reading_output = false
  end

  def root
    @filetree["/"]
  end
end

class AocFile
  def initialize(name, info)
    @name = name
    if info == "dir"
      @dir = true
      @tree = {}
      @size = 0
    else
      @dir = false
      @size = info.to_i
    end
  end

  attr_reader :name, :tree
  attr_accessor :size

  def dir?
    @dir
  end
end

f = File.open("./input.txt", "r")

root = AocFile.new("/", "dir")
state = State.new(root)

f.each_line do |l|
  state.process_line(l)
end

if f.eof?
  state.process_output
end

TOTAL_FILESYSTEM_SIZE = 70000000
MINIMUM_NEEDED_FOR_UPDATE = 30000000
FILE_SIZE_THRESHOLD = 100_000

puts "Part 1: #{state.all_files
  .select { |f| f.size < FILE_SIZE_THRESHOLD && f.dir? }
  .map(&:size)
  .sum}"

puts "===="

unused_space = TOTAL_FILESYSTEM_SIZE - state.root.size
needed = MINIMUM_NEEDED_FOR_UPDATE - unused_space
puts "Unused space: #{unused_space}"
puts "Needed: #{needed}"
puts "Part 2: #{state.all_files
  .select { |f| f.dir? && f.size >= needed}
  .map(&:size)
  .min}"
