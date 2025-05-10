const converter = new showdown.Converter();
const username = 'MilesONerd';
const repo = 'website';
const folder = 'posts';
const branch = 'main';

const postList = document.getElementById('post-list');
const contentDiv = document.getElementById('content');

async function fetchPostList() {
    const url = `https://api.github.com/repos/${username}/${repo}/contents/blog/${folder}?ref=${branch}`;

    try {
        const response = await fetch(url);
        if (!response.ok) throw new Error('Error loading post list.');

        const files = await response.json();
        const mdFiles = files.filter(file => file.name.endsWith('.md'));

        postList.innerHTML = '';

        if (mdFiles.length === 0) {
            postList.innerHTML = '<li>No posts available.</li>';
        }

        mdFiles.forEach(file => {
            const listItem = document.createElement('li');
            listItem.innerHTML = `<a href="#" data-url="${file.download_url}">${file.name.replace('.md', '')}</a>`;
            postList.appendChild(listItem);
        });

        postList.addEventListener('click', (e) => {
            if (e.target.tagName.toLowerCase() === 'a') {
                e.preventDefault();
                const postUrl = e.target.getAttribute('data-url');
                loadPostContent(postUrl);
            }
        });

    } catch (error) {
        console.error(error);
        postList.innerHTML = '<li>Error loading posts 😢</li>';
    }
}

async function loadPostContent(url) {
    try {
        const response = await fetch(url);
        if (!response.ok) throw new Error('Error loading post.');

        const markdown = await response.text();
        const htmlContent = converter.makeHtml(markdown);
        contentDiv.innerHTML = htmlContent;
    } catch (error) {
        console.error(error);
        contentDiv.innerHTML = '<p>Error loading post content 😢</p>';
    }
}

// Carregar lista de posts quando a página carregar
document.addEventListener('DOMContentLoaded', fetchPostList);
