use std::cmp::Reverse;

use graphs::WeightedGraph;
use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;

// Returns whether number `a` is closer to `to` than `b`.
fn closer_to(
  a: OrderedFloat<f32>,
  b: OrderedFloat<f32>,
  to: f32,
) -> bool {
  (a - to).abs() < (b - to).abs()
}

pub fn approximate_cost_search(
  graph: &WeightedGraph,
  start: usize,
  end: usize,
  cost: f32,
) -> Option<Vec<usize>> {
  let mut distance =
    vec![OrderedFloat::from(f32::MAX); graph.size];
  distance[start] = OrderedFloat::from(0.0);

  let mut predeccessor = vec![usize::MAX; graph.size];

  let mut queue = PriorityQueue::<_, _>::from_iter(
    (0..graph.size)
      .map(|index| (index, Reverse(distance[index]))),
  );

  while let Some((vertex, Reverse(current_distance))) =
    queue.pop()
  {
    for &(neighbor, weight) in graph.get_neighbors(vertex) {
      let new_distance = current_distance + weight;

      if (neighbor == end
        && closer_to(new_distance, current_distance, cost))
        || new_distance < distance[neighbor]
      {
        distance[neighbor] = new_distance;
        queue.change_priority(
          &neighbor,
          Reverse(new_distance),
        );
        predeccessor[neighbor] = vertex;
      }
    }
  }

  if predeccessor[end] != usize::MAX {
    let mut path = vec![];
    let mut current = end;

    while current != usize::MAX {
      path.push(current);
      current = predeccessor[current];
    }

    path.reverse();

    Some(path)
  } else {
    None
  }
}
