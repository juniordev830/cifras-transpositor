extends RefCounted
static func can_build(point: Vector2, towers: Array, gold: int) -> bool:
    if gold < 20 or point.x < 55 or point.x > 905 or point.y < 140 or point.y > 550 or absf(point.y - 320) < 65:
        return false
    for tower in towers:
        if point.distance_to(tower.position) < 50:
            return false
    return true
static func enemy_health(index: int) -> int:
    return 2 + int(index / 10.0)
