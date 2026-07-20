#!/usr/bin/env bash

TODO_FILE="TODO.md"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m' # No Color

YES_FLAG=false

usage() {
    echo "Usage: $0 [command] [args]"
    echo ""
    echo "Commands:"
    echo "  list, ls          List all todos (default)"
    echo "  add, + [text]     Add a new todo"
    echo "  done, d [i|text]  Mark a todo as done"
    echo "  wip, p [i|text]   Mark a todo as in-progress"
    echo "  edit, e [i] [text] Edit a todo"
    echo "  rm, - [i|text]    Remove a todo"
    echo "  search [query]    Search todos"
    echo "  stats             Show statistics"
    echo "  init              Initialize a new todo list"
    echo "  help              Show this help message"
    echo ""
    echo "Options:"
    echo "  -y, --yes         Skip confirmation prompts"
    echo ""
    echo "Examples:"
    echo "  $0 add \"Code a new feature\""
    echo "  $0 add --priority high \"Fix critical bug\""
    echo "  $0 done 0"
    echo "  $0 edit 1 \"Updated task description\""
    echo "  $0 rm 2"
    echo "  $0 search \"feature\""
    echo "  $0 stats"
    echo ""
    echo "Indexing: Use 0-indexed numbers as shown in list output."
    echo "  Example: '$0 done 0' marks the first todo as done."
}

confirm() {
    if [[ "$YES_FLAG" == true ]]; then
        return 0
    fi
    local prompt="$1"
    read -r -p "$prompt [y/N] " response
    case "$response" in
        [yY][eE][sS]|[yY]) return 0 ;;
        *) return 1 ;;
    esac
}

# Helper: Get the line content at a 0-indexed position
get_line_at_index() {
    local index="$1"
    local line_num=$((index + 1))
    sed -n "${line_num}p" "$TODO_FILE"
}

# Helper: Get total number of todo lines (excluding comments and blank lines)
get_total_todos() {
    local count=0
    local in_comment=false
    while IFS= read -r line; do
        # Trim leading/trailing whitespace
        line="${line#"${line%%[![:space:]]*}"}"
        line="${line%"${line##*[![:space:]]}"}"
        if [[ "$line" =~ ^\<\!-- ]]; then
            if [[ ! "$line" =~ --\>$ ]]; then
                in_comment=true
            fi
            continue
        fi
        if [[ "$in_comment" == true ]]; then
            if [[ "$line" =~ --\>$ ]]; then
                in_comment=false
            fi
            continue
        fi
        if [[ -n "$line" ]]; then
            count=$((count + 1))
        fi
    done < "$TODO_FILE"
    echo "$count"
}

# Helper: Check if argument is a number
is_number() {
    [[ "$1" =~ ^[0-9]+$ ]]
}

# Helper: Find line number (1-indexed) by index (0-indexed)
find_line_by_index() {
    local target_index="$1"
    local current_index=0
    local in_comment=false
    local line_num=0
    while IFS= read -r line; do
        line_num=$((line_num + 1))
        # Trim leading/trailing whitespace
        line="${line#"${line%%[![:space:]]*}"}"
        line="${line%"${line##*[![:space:]]}"}"
        if [[ "$line" =~ ^\<\!-- ]]; then
            if [[ ! "$line" =~ --\>$ ]]; then
                in_comment=true
            fi
            continue
        fi
        if [[ "$in_comment" == true ]]; then
            if [[ "$line" =~ --\>$ ]]; then
                in_comment=false
            fi
            continue
        fi
        if [[ -n "$line" ]]; then
            if [[ "$current_index" -eq "$target_index" ]]; then
                echo "$line_num"
                return 0
            fi
            current_index=$((current_index + 1))
        fi
    done < "$TODO_FILE"
    return 1
}

# Helper: Find line number (1-indexed) by text search
find_line_by_text() {
    local search_term="$1"
    local in_comment=false
    local line_num=0
    while IFS= read -r line; do
        line_num=$((line_num + 1))
        local trimmed
        # Trim leading/trailing whitespace
        trimmed="${line#"${line%%[![:space:]]*}"}"
        trimmed="${trimmed%"${trimmed##*[![:space:]]}"}"
        if [[ "$trimmed" =~ ^\<\!-- ]]; then
            if [[ ! "$trimmed" =~ --\>$ ]]; then
                in_comment=true
            fi
            continue
        fi
        if [[ "$in_comment" == true ]]; then
            if [[ "$trimmed" =~ --\>$ ]]; then
                in_comment=false
            fi
            continue
        fi
        if [[ -n "$trimmed" && "$trimmed" == *"$search_term"* ]]; then
            echo "$line_num"
            return 0
        fi
    done < "$TODO_FILE"
    return 1
}

# Parse arguments for --yes flag
parse_flags() {
    ARGS=()
    for arg in "$@"; do
        case "$arg" in
            -y|--yes) YES_FLAG=true ;;
            *) ARGS+=("$arg") ;;
        esac
    done
}

# Find a todo by index or text, returns line number
find_todo() {
    local arg="$1"
    if is_number "$arg"; then
        local line_num
        line_num=$(find_line_by_index "$arg")
        if [[ -z "$line_num" ]]; then
            echo -e "${RED}Error: No todo at index $arg.${NC}" >&2
            return 1
        fi
        echo "$line_num"
    else
        local line_num
        line_num=$(find_line_by_text "$arg")
        if [[ -z "$line_num" ]]; then
            echo -e "${RED}Todo not found: $arg${NC}" >&2
            return 1
        fi
        echo "$line_num"
    fi
}

# Get status marker and content from a line
parse_todo_line() {
    local line="$1"
    if [[ "$line" =~ ^\[(x)\] ]]; then
        echo "done ${line#\[*\] }"
    elif [[ "$line" =~ ^\[-\] ]]; then
        echo "wip ${line#\[*\] }"
    elif [[ "$line" =~ ^\[\!\!\] ]]; then
        echo "urgent ${line#\[*\] }"
    elif [[ "$line" =~ ^\[\!\] ]]; then
        echo "high ${line#\[*\] }"
    elif [[ "$line" =~ ^\[\] ]]; then
        echo "pending ${line#\[*\] }"
    else
        echo "other $line"
    fi
}

# Main command parsing
parse_flags "$@"
set -- "${ARGS[@]}"

command="$1"
shift || true

if [[ -z "$command" ]]; then
    command="list"
fi

if [[ "$command" != "init" && "$command" != "help" && ! -f "$TODO_FILE" ]]; then
    echo -e "${RED}Error: $TODO_FILE not found.${NC}"
    echo "To create it run: $0 init"
    exit 1
fi

case "$command" in
    help)
        usage
        ;;
    add|+)
        if [[ -z "$*" ]]; then
            echo -e "${RED}Error: No todo text provided.${NC}"
            echo "Usage: $0 add [text]"
            exit 1
        fi
        # Check for priority flag
        priority=""
        todo_text=""
        while [[ $# -gt 0 ]]; do
            case "$1" in
                --priority|-p)
                    shift
                    case "$1" in
                        high|!) priority="!" ;;
                        urgent|!!) priority="!!" ;;
                        *) priority="" ;;
                    esac
                    shift
                    ;;
                *)
                    todo_text="$1"
                    shift
                    ;;
            esac
        done
        if [[ -z "$todo_text" ]]; then
            echo -e "${RED}Error: No todo text provided.${NC}"
            exit 1
        fi
        if [[ -n "$priority" ]]; then
            echo "[$priority] $todo_text" >> "$TODO_FILE"
        else
            echo "[] $todo_text" >> "$TODO_FILE"
        fi
        total=$(get_total_todos)
        echo -e "${GREEN}Added:${NC} $todo_text (index: $((total - 1)))"
        ;;
    done|finish|d)
        if [[ -z "$*" ]]; then
            echo -e "${RED}Error: No todo specified.${NC}"
            echo "Usage: $0 done [index|text]"
            exit 1
        fi
        line_num=$(find_todo "$1") || exit 1
        current_line=$(sed -n "${line_num}p" "$TODO_FILE")
        if [[ "$current_line" =~ ^\[x\] ]]; then
            echo -e "${YELLOW}Todo already marked as done.${NC}"
            exit 0
        fi
        content="${current_line#\[*\] }"
        if confirm "Mark '$content' as done?"; then
            sed -i "${line_num}s/\[.*\]/[x]/" "$TODO_FILE"
            echo -e "${GREEN}Marked as done:${NC} $content"
        fi
        ;;
    wip|doing|p|partial)
        if [[ -z "$*" ]]; then
            echo -e "${RED}Error: No todo specified.${NC}"
            echo "Usage: $0 wip [index|text]"
            exit 1
        fi
        line_num=$(find_todo "$1") || exit 1
        current_line=$(sed -n "${line_num}p" "$TODO_FILE")
        if [[ "$current_line" =~ ^\[-\] ]]; then
            echo -e "${YELLOW}Todo already marked as in-progress.${NC}"
            exit 0
        fi
        content="${current_line#\[*\] }"
        sed -i "${line_num}s/\[.*\]/[-]/" "$TODO_FILE"
        echo -e "${GREEN}Marked as in-progress:${NC} $content"
        ;;
    edit|e)
        if [[ $# -lt 2 ]]; then
            echo -e "${RED}Error: Usage: $0 edit [index] [new text]${NC}"
            exit 1
        fi
        if ! is_number "$1"; then
            echo -e "${RED}Error: First argument must be a numeric index.${NC}"
            exit 1
        fi
        idx="$1"
        shift
        new_text="$*"
        line_num=$(find_line_by_index "$idx")
        if [[ -z "$line_num" ]]; then
            echo -e "${RED}Error: No todo at index $idx.${NC}"
            exit 1
        fi
        current_line=$(sed -n "${line_num}p" "$TODO_FILE")
        current_content="${current_line#\[*\] }"
        echo -e "Current: $current_content"
        echo -e "New:     $new_text"
        if confirm "Update this todo?"; then
            # Preserve the status marker
            marker=$(echo "$current_line" | grep -o '^\[.*\]')
            sed -i "${line_num}s/.*/${marker} ${new_text}/" "$TODO_FILE"
            echo -e "${GREEN}Updated todo at index ${idx}.${NC}"
        fi
        ;;
    rm|delete|-)
        if [[ -z "$*" ]]; then
            echo -e "${RED}Error: No todo specified.${NC}"
            echo "Usage: $0 rm [index|text]"
            exit 1
        fi
        line_num=$(find_todo "$1") || exit 1
        current_line=$(sed -n "${line_num}p" "$TODO_FILE")
        content="${current_line#\[*\] }"
        if confirm "Remove '$content'?"; then
            sed -i "${line_num}d" "$TODO_FILE"
            echo -e "${GREEN}Removed:${NC} $content"
        fi
        ;;
    search|s)
        if [[ -z "$*" ]]; then
            echo -e "${RED}Error: No search query provided.${NC}"
            echo "Usage: $0 search [query]"
            exit 1
        fi
        query="$*"
        echo -e "Search results for: ${BOLD}$query${NC}"
        echo "--------------------------------"
        in_comment=false
        line_num=0
        display_index=0
        while IFS= read -r line; do
            line_num=$((line_num + 1))
            # Trim leading/trailing whitespace
            trimmed="${line#"${line%%[![:space:]]*}"}"
            trimmed="${trimmed%"${trimmed##*[![:space:]]}"}"
            if [[ "$trimmed" =~ ^\<\!-- ]]; then
                if [[ ! "$trimmed" =~ --\>$ ]]; then
                    in_comment=true
                fi
                continue
            fi
            if [[ "$in_comment" == true ]]; then
                if [[ "$trimmed" =~ --\>$ ]]; then
                    in_comment=false
                fi
                continue
            fi
            if [[ -n "$trimmed" && "$trimmed" == *"$query"* ]]; then
                parsed=$(parse_todo_line "$trimmed")
                status="${parsed%% *}"
                content="${parsed#* }"
                case "$status" in
                    done) printf "${GREEN}%d ✔ %s${NC}\n" "$display_index" "$content" ;;
                    wip) printf "${YELLOW}%d ◐ %s${NC}\n" "$display_index" "$content" ;;
                    urgent) printf "${RED}%d !! %s${NC}\n" "$display_index" "$content" ;;
                    high) printf "${RED}%d ! %s${NC}\n" "$display_index" "$content" ;;
                    pending) printf "${BOLD}%d ☐ %s${NC}\n" "$display_index" "$content" ;;
                    *) printf "${BLUE}%d %s${NC}\n" "$display_index" "$content" ;;
                esac
            fi
            if [[ -n "$trimmed" ]]; then
                display_index=$((display_index + 1))
            fi
        done < "$TODO_FILE"
        ;;
    stats)
        done_count=0
        in_progress_count=0
        pending_count=0
        in_comment=false
        while IFS= read -r line; do
            # Trim leading/trailing whitespace
            line="${line#"${line%%[![:space:]]*}"}"
            line="${line%"${line##*[![:space:]]}"}"
            if [[ "$line" =~ ^\<\!-- ]]; then
                if [[ ! "$line" =~ --\>$ ]]; then
                    in_comment=true
                fi
                continue
            fi
            if [[ "$in_comment" == true ]]; then
                if [[ "$line" =~ --\>$ ]]; then
                    in_comment=false
                fi
                continue
            fi
            if [[ -n "$line" ]]; then
                if [[ "$line" =~ ^\[x\] ]]; then
                    done_count=$((done_count + 1))
                elif [[ "$line" =~ ^\[-\] ]]; then
                    in_progress_count=$((in_progress_count + 1))
                else
                    pending_count=$((pending_count + 1))
                fi
            fi
        done < "$TODO_FILE"
        total=$((done_count + in_progress_count + pending_count))
        echo -e "${BOLD}Todo Statistics${NC}"
        echo "--------------------------------"
        echo -e "Total:       ${BOLD}$total${NC}"
        echo -e "Done:        ${GREEN}$done_count${NC}"
        echo -e "In-progress: ${YELLOW}$in_progress_count${NC}"
        echo -e "Pending:     ${BLUE}$pending_count${NC}"
        if [[ "$total" -gt 0 ]]; then
            pct=$((done_count * 100 / total))
            echo "--------------------------------"
            echo -e "Progress:    ${BOLD}${pct}%${NC}"
        fi
        ;;
    init)
        if [[ -f "$TODO_FILE" ]]; then
            echo -e "${RED}Error: $TODO_FILE already exists.${NC}"
            exit 1
        else
            echo "<!-- Todo list generated and controlled by todos.sh
For more information run: $0 help
-->" > "$TODO_FILE"
            echo -e "${GREEN}Initialized $TODO_FILE${NC}"
        fi
        ;;
    list|ls)
        filter="all"
        for arg in "$@"; do
            case "$arg" in
                -a|--all) filter="all" ;;
                -p|--pending) filter="pending" ;;
                -d|--done) filter="done" ;;
                -ip|--in-progress) filter="in-progress" ;;
            esac
        done

        echo -e "${BOLD}Todos:${NC}"
        echo "--------------------------------"
        in_comment=false
        line_num=0
        display_index=0
        shown=0
        while IFS= read -r line; do
            line_num=$((line_num + 1))
            # Trim leading/trailing whitespace
            trimmed="${line#"${line%%[![:space:]]*}"}"
            trimmed="${trimmed%"${trimmed##*[![:space:]]}"}"
            if [[ "$trimmed" =~ ^\<\!-- ]]; then
                if [[ ! "$trimmed" =~ --\>$ ]]; then
                    in_comment=true
                fi
                continue
            fi
            if [[ "$in_comment" == true ]]; then
                if [[ "$trimmed" =~ --\>$ ]]; then
                    in_comment=false
                fi
                continue
            fi
            if [[ -n "$trimmed" ]]; then
                show=false
                if [[ "$filter" == "all" ]]; then
                    show=true
                elif [[ "$filter" == "pending" && ! "$trimmed" =~ ^\[x\] && ! "$trimmed" =~ ^\[-\] ]]; then
                    show=true
                elif [[ "$filter" == "done" && "$trimmed" =~ ^\[x\] ]]; then
                    show=true
                elif [[ "$filter" == "in-progress" && "$trimmed" =~ ^\[-\] ]]; then
                    show=true
                fi
                if [[ "$show" == true ]]; then
                    parsed=$(parse_todo_line "$trimmed")
                    status="${parsed%% *}"
                    content="${parsed#* }"
                    case "$status" in
                        done) printf "${GREEN}%d ✔ %s${NC}\n" "$display_index" "$content" ;;
                        wip) printf "${YELLOW}%d ◐ %s${NC}\n" "$display_index" "$content" ;;
                        urgent) printf "${RED}%d !! %s${NC}\n" "$display_index" "$content" ;;
                        high) printf "${RED}%d ! %s${NC}\n" "$display_index" "$content" ;;
                        pending) printf "${BOLD}%d ☐ %s${NC}\n" "$display_index" "$content" ;;
                        *) printf "${BLUE}%d %s${NC}\n" "$display_index" "$content" ;;
                    esac
                    shown=$((shown + 1))
                fi
                display_index=$((display_index + 1))
            fi
        done < "$TODO_FILE"
        if [[ "$shown" -eq 0 ]]; then
            echo -e "${BLUE}No todos to display.${NC}"
        fi
        ;;
    *)
        echo -e "${RED}Unknown command: $command${NC}"
        echo "Run '$0 help' for usage information."
        exit 1
        ;;
esac
