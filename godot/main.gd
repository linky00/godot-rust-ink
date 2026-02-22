extends Node

@onready var ink_story_instance = $InkStoryInstance

func _ready() -> void:
	print(ink_story_instance.cont())
