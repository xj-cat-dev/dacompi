#include "wow_gameplay_bridge/lib.h"
#include <ios>
#include <iostream>

int main() {
  const bool later = wow::gameplay::ends_at_or_after(1200, 1000);
  const bool earlier = wow::gameplay::ends_at_or_after(800, 1000);

  std::cout << std::boolalpha << later << '\n' << earlier << '\n';

  const auto cooldown = wow::gameplay::query_cooldown(3000, 1000, 5000);

  std::cout << std::boolalpha << "active=" << cooldown.active
            << " start=" << cooldown.start << " duration=" << cooldown.duration
            << '\n';

  const bool cooldown_ok =
      cooldown.active && cooldown.start == 1000 && cooldown.duration == 5000;

  return (later && !earlier && cooldown_ok) ? 0 : 1;
}
