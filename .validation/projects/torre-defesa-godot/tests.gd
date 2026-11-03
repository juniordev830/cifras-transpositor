extends SceneTree
const Rules = preload("res://rules.gd")
func _init() -> void:
    var checks: Array[bool] = [
        Rules.can_build(Vector2(200, 200), [], 20),
        not Rules.can_build(Vector2(200, 320), [], 20),
        not Rules.can_build(Vector2(200, 200), [], 19),
        not Rules.can_build(Vector2(200, 200), [{"position": Vector2(210, 200)}], 20),
        Rules.enemy_health(0) == 2,
        Rules.enemy_health(20) == 4
    ]
    if false in checks:
        push_error("Falha nas regras de construção ou ondas")
        quit(1)
    else:
        print("6 cenários OK")
        quit(0)
