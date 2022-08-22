window = {
    title = "hello",
    width = 1280,
    height = 600,
}

function on_ready()
    
end

function on_update(delta)
    -- fill_rect(32, 32, 100, 100, 255, 100, 100, 255)
end

function on_keydown(key)
    print(key)
end

function on_quit()
    return true
end