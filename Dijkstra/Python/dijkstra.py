# Impletmentation of Dijkstra's algorithm
# Remember
#   - It works on weigthed graphs (Not like DSF-BFS algorithms)
#   - It does not work on negative weighted graphs

# Let's use 3 dictionaries
#   - Grapgh
#   - Costs
#   - Parent

from time import sleep


graph = {
    'START': {
        'A': 6,
        'B': 2
    },
    'A': {
        'FIN': 1
    },
    'B': {
        'A': 3,
        'FIN': 5
    },
    'FIN': {}
}

costs = {
    'A': 6,
    'B': 2,
    'FIN': float("inf")
}

parents = {
    'A': 'START',
    'B': 'START',
    'FIN': None
}

# Also, we need an array that records all the visited nodes.
visited = list()

def find_cheapest_node(costs):
    """
        Function that returns the node with the lowest
        value of cost and has not been visted.
    """
    lowest_cost_node = None
    lowest_cost = float('inf')
    # sleep(5)
    for node in costs:
        # print(f"costs[{node}]")
        if (costs[node] < lowest_cost) and (node not in visited):
            lowest_cost_node = node
            lowest_cost = costs[node]
    return lowest_cost_node


def path():
    path_start2finish = list()
    parent_node = parents['FIN']
    while parent_node is not 'START':
        path_start2finish.append(parent_node)
        parent_node = parents[parent_node]

    return ['START'] + path_start2finish[::-1] + ['FINISH']


def dijkstras_algorithm():
    node = find_cheapest_node(costs)
    while node is not None:
        # print(f"\n\nLowest node: {node}")
        # print(f"Visited nodes {visited}")
        for neighbor in graph[node].keys():
            cost_to_this_node = costs[node] + graph[node][neighbor]
            if cost_to_this_node < costs[neighbor]:
                parents[neighbor] = node
                costs[neighbor] = cost_to_this_node
        visited.append(node)
        
        node = find_cheapest_node(costs)
        # sleep(6)
    return path()


print('Graph')
print(graph, end='\n\n')

print("=" * 80)
print("=" * 80)

print('Costs')
print(costs, end='\n\n')

print("=" * 80)
print("=" * 80)

print('Parents')
print(parents)

print("=" * 80)
print("=" * 80)

print("Final route")
print(dijkstras_algorithm())
