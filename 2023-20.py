from dataclasses import dataclass
from itertools import count
from math import lcm
from queue import Queue
from collections import defaultdict
import graphviz

module_table = {}
q = Queue()
dot = graphviz.Digraph(format='png')
shape_table = {'broadcaster': 'star', 'flipflop': 'box', 'conjunction': 'ellipse'}

@dataclass()
class Module:
    name: str
    type: str
    inputs: list
    outputs: list
    flip_state: bool
    conjunction_memory: defaultdict
    def __init__(self, name, m_type):
        self.name = name
        self.type = m_type
        self.inputs = []
        self.outputs = []
        self.flip_state = False
        self.conjunction_memory = defaultdict(bool)
        dot.node(name, shape=shape_table[m_type])
    def __repr__(self):
        return self.name
    def pulse(self, value, sender):
        if self.type == 'broadcaster':
            for output in self.outputs:
                q.put((output, value, self.name))
        elif self.type == 'flipflop':
            if not value:
                self.flip_state = not self.flip_state
                for output in self.outputs:
                    q.put((output, self.flip_state, self.name))
        elif self.type == 'conjunction':
            self.conjunction_memory[sender] = value
            output_value = not all(self.conjunction_memory[input_module.name] for input_module in self.inputs)
            for output in self.outputs:
                q.put((output, output_value, self.name))

with open('input.txt', 'r') as f:
    # first pass - just create all modules
    for line in f.readlines():
        left, _ = line.strip().split(' -> ')
        if left.startswith('%'):
            module_table[left[1:]] = Module(left[1:], 'flipflop')
        elif left.startswith('&'):
            module_table[left[1:]] = Module(left[1:], 'conjunction')
        else:
            module_table[left] = Module('broadcaster', 'broadcaster')
with open('input.txt', 'r') as f:
    # second pass - link modules to their inputs, outputs
    for line in f.readlines():
        left, right = line.strip().split(' -> ')
        m = module_table['broadcaster'] if left == 'broadcaster' else module_table[left[1:]]
        for output in right.split(', '):
            if output not in module_table:
                module_table[output] = Module(output, 'broadcaster')
            dot.edge(m.name, output)
            output_module = module_table[output]
            m.outputs.append(output_module)
            output_module.inputs.append(m)

# part 1
broadcaster = module_table['broadcaster']
low_pulses = 0
high_pulses = 0
def press_button(count):
    global low_pulses, high_pulses
    q.put((broadcaster, False, None))
    while not q.empty():
        module, pulse_value, sender = q.get()
        if pulse_value:
            high_pulses += 1
        else:
            low_pulses += 1
        # if module.name in ['xt', 'mk', 'zc', 'fp'] and not pulse_value:
        #     print(module.name, count, int(not module.flip_state))
        module.pulse(pulse_value, sender)
    return False

# part 1
for i in range(1000):
    press_button(i)
print(low_pulses * high_pulses)

# # part 2
# for presses in count(1):
#     press_button(presses)

# The following numbers found by just running button presses manually
# and printing the intervals at which xt, mk, zc and fp get triggered

# 4007, 3931, 3923, 3767
# print(lcm(4007, 3931, 3923, 3767))

# # for drawing the graph
# dot.render()

