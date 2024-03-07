import { h, render, ref } from 'vue'
import message from './message.vue';
import messageBox from './messageBox.vue'

const div = document.createElement('div');
document.body.appendChild(div);
const messageArr = ref([]);

div.style.position = 'absolute';
div.style.top = '30px';
div.style.right = '0';
div.style.padding = '5px 10px';
div.style.maxHeight = 'calc(100vh - 10px)';
div.style.overflow = 'hidden';
div.style.zIndex = '99';

function getMessageIndex(id) {
    for (let i = 0; i < messageArr.value.length; i++) {
        if (messageArr.value[i].props.id == id) {
            return i;
        }
    }
}

let id = 0;
export default function Message(object) {
    if (id == 0) {
        let vnode = h(messageBox, { messageArr }, null);
        render(vnode, div);
    }
    // render(null, div);
    object.id = id++;
    object.close = () => {
        let index = getMessageIndex(object.id);
        messageArr.value.splice(index, 1);
        // vnode = h(messageBox, null, () => messageArr);
        // render(vnode, div);
    }
    if (object.duration) {
        setTimeout(() => {
            object.close();
        }, object.duration);
    }
    messageArr.value.push(h(message, object, null));
}