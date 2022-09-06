#pragma once

#include <cinttypes>
#include <cstdlib>

template<typename T>
class PointerMap {
public:
    static const size_t HOOK_COUNT = 0x400;

    struct Node {
        uintptr_t key;
        T data;
    };

private:
    Node table[HOOK_COUNT];

public:
    PointerMap() : table { Node { .key = 0 } } {}
    
    const T* Get(uintptr_t key) const {
        size_t hash = static_cast<size_t>(key) % HOOK_COUNT;
        const T* out_data = nullptr;
        while (table[hash].key != 0) {
            if (table[hash].key == key) {
                out_data = &table[hash].data;
                break;
            }

            hash = (hash + 1) % HOOK_COUNT;
        }

        return out_data;
    }

    T* GetMut(uintptr_t key) {
        size_t hash = static_cast<size_t>(key) % HOOK_COUNT;
        T* out_data = nullptr;
        
        while (table[hash].key != 0) {
            if (table[hash].key == key) {
                out_data = &table[hash].data;
                break;
            }

            hash = (hash + 1) % HOOK_COUNT;
        }

        return out_data;
    }

    bool Insert(uintptr_t key, T data) {
        size_t hash = static_cast<size_t>(key) % HOOK_COUNT;
        size_t current = hash;
        do {
            if (table[current].key == 0) {
                table[current].key = key;
                table[current].data = data;
                return true;
            }
            current = (current + 1) % HOOK_COUNT;
        } while (current != hash);
        return false;
    }
};