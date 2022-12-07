class State
  def initialize(root)
    @pwd = root.name
    @dir_stack = []
    @filetree = {root.name => root}
    @reading_output = false
    @output = []
    @all_files = []
  end

  attr_reader :filetree, :output, :all_files
  attr_accessor :reading_output

  def reading_output?
    reading_output
  end

  def cd(dest)
    if dest == ".."
      @dir_stack.pop
    else
      @dir_stack.push(dest)
    end
    @pwd = dest
  end

  def process_output
    @output.each do |o|
      info, name = o.split(" ").map(&:strip)
      file = AocFile.new(name, info)

      # insert files
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
  case true # match each case with true === <boolean expression>
  when l.start_with?("$ cd")
    state.process_output if state.reading_output?
    dest = l[5..].strip
    state.cd(dest);
    next
  when l.start_with?("$ ls")
    state.reading_output = true;
    next
  when state.reading_output? && !l.start_with?("$")
    state.output.push(l.strip);
    next
  end
end

if f.eof?
  state.process_output
end

TOTAL_FILESYSTEM_SIZE = 70000000
MINIMUM_NEEDED_FOR_UPDATE = 30000000

p "Part 1: #{state.all_files.select { |f| f.size < 100000 && f.dir? }.map(&:size).sum}"
unused_space = TOTAL_FILESYSTEM_SIZE - state.filetree["/"].size
needed = MINIMUM_NEEDED_FOR_UPDATE - unused_space
p "===="
p "Unused space: #{unused_space}"
p "Needed: #{needed}"
p "Part 2: #{state.all_files.select { |f| f.dir? && f.size >= needed}.map(&:size).min}"
