extends Node2D
const Rules = preload("res://rules.gd")
var towers: Array = []
var enemies: Array = []
var shots: Array = []
var gold: int = 60
var lives: int = 10
var spawned: int = 0
var timer: float = 0.0
var selected: int = -1
var finished: bool = false

func reset() -> void:
    towers.clear()
    enemies.clear()
    gold = 60
    lives = 10
    spawned = 0
    timer = 0.0
    selected = -1
    finished = false

func _input(event: InputEvent) -> void:
    if event is InputEventKey and event.pressed and not event.echo:
        if event.keycode == KEY_R:
            reset()
        if event.keycode == KEY_ESCAPE:
            get_tree().quit()
        if event.keycode == KEY_U and not finished and selected >= 0 and gold >= 15 and towers[selected].level < 3:
            towers[selected].level += 1
            gold -= 15
    if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT and not finished:
        var point: Vector2 = event.position
        for i in range(towers.size()):
            if point.distance_to(towers[i].position) < 24:
                selected = i
                return
        if Rules.can_build(point, towers, gold):
            towers.append({"position": point, "cooldown": 0.0, "level": 1})
            selected = towers.size() - 1
            gold -= 20

func _process(delta: float) -> void:
    shots.clear()
    if finished:
        queue_redraw()
        return
    timer += delta
    if timer >= 1.2 and spawned < 30:
        timer -= 1.2
        enemies.append({"position": Vector2(0, 320), "hp": Rules.enemy_health(spawned), "speed": 48.0 + spawned})
        spawned += 1
    for tower in towers:
        tower.cooldown -= delta
        if tower.cooldown <= 0:
            for enemy in enemies:
                if enemy.hp > 0 and tower.position.distance_to(enemy.position) < 165:
                    enemy.hp -= tower.level
                    shots.append([tower.position, enemy.position])
                    tower.cooldown = 0.65
                    break
    for i in range(enemies.size() - 1, -1, -1):
        var enemy = enemies[i]
        if enemy.hp <= 0:
            gold += 5
            enemies.remove_at(i)
        else:
            enemy.position.x += enemy.speed * delta
            if enemy.position.x > 960:
                lives -= 1
                enemies.remove_at(i)
    finished = lives <= 0 or (spawned == 30 and enemies.is_empty())
    queue_redraw()

func label_at(text: String, position: Vector2, size: int = 20, color: Color = Color.WHITE) -> void:
    draw_string(ThemeDB.fallback_font, position, text, HORIZONTAL_ALIGNMENT_LEFT, -1, size, color)

func _draw() -> void:
    for x in range(0, 960, 40):
        draw_line(Vector2(x, 135), Vector2(x, 560), Color(0.07, 0.12, 0.18))
    for y in range(135, 560, 40):
        draw_line(Vector2(0, y), Vector2(960, y), Color(0.07, 0.12, 0.18))
    draw_rect(Rect2(0, 285, 960, 70), Color(0.14, 0.18, 0.24))
    draw_line(Vector2(0, 320), Vector2(960, 320), Color(0.4, 0.45, 0.5), 2)
    label_at("TORRE DE DEFESA / RESISTA A 30 INIMIGOS", Vector2(28, 42), 26, Color(0.3, 0.95, 0.8))
    label_at("Ouro: %d     Base: %d     Inimigos: %d/30" % [gold, lives, spawned], Vector2(28, 90))
    for i in range(towers.size()):
        var tower = towers[i]
        if i == selected:
            draw_arc(tower.position, 165, 0, TAU, 80, Color(0.2, 0.6, 0.6, 0.3), 2)
        draw_circle(tower.position, 21, Color(0.2, 0.8, 0.75))
        label_at(str(tower.level), tower.position + Vector2(-6, 7), 20, Color(0.02, 0.1, 0.12))
    for enemy in enemies:
        draw_circle(enemy.position, 14, Color(1.0, 0.35, 0.35))
        label_at(str(enemy.hp), enemy.position + Vector2(-5, -20), 16)
    for shot in shots:
        draw_line(shot[0], shot[1], Color(0.8, 1.0, 0.5), 3)
    label_at("Clique: torre (20) / selecionar     U: melhorar (15)     R: reiniciar     Esc: sair", Vector2(25, 605), 18)
    if finished:
        draw_rect(Rect2(200, 220, 560, 150), Color(0.02, 0.04, 0.08, 0.96))
        label_at("VITORIA!" if lives > 0 else "BASE DESTRUIDA", Vector2(325, 285), 30)
        label_at("Pressione R para tentar novamente", Vector2(285, 330), 20)
